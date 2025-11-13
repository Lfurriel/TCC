// Script otimizado para processar métricas do K6 - Resolve limite de string
const fs = require('fs');
const path = require('path');
const readline = require('readline');

class OptimizedMetricsProcessor {
    constructor() {
        this.resultsDir = './results';
        this.rawJsonPath = path.join(this.resultsDir, 'raw.json');
        this.csvFile = path.join(this.resultsDir, 'metrics.csv');

        this.metricsBuffer = [];
        this.intervalSeconds = 5;
        this.startTime = null;
        this.processedIntervals = 0;
        this.totalLinesProcessed = 0;
        this.lastFilePosition = 0;

        console.log('Iniciando processador de métricas otimizado...');
        console.log(`Intervalo: ${this.intervalSeconds}s`);

        this.ensureResultsDir();
        this.initCSV();
        this.waitForRawFile();
    }

    ensureResultsDir() {
        if (!fs.existsSync(this.resultsDir)) {
            fs.mkdirSync(this.resultsDir, { recursive: true });
        }
    }

    waitForRawFile() {
        const checkInterval = 1000;
        const checkFile = () => {
            if (fs.existsSync(this.rawJsonPath)) {
                const stats = fs.statSync(this.rawJsonPath);
                if (stats.size > 0) {
                    console.log(`Arquivo encontrado: ${this.rawJsonPath} (${(stats.size / 1024 / 1024).toFixed(2)} MB)`);
                    this.startProcessing();
                    return;
                }
            }
            setTimeout(checkFile, checkInterval);
        };
        checkFile();
    }

    initCSV() {
        const headers = [
            'elapsed_seconds',
            'virtual_users',
            'check_2xx_success',
            'check_2xx_fail',
            'check_response_success',
            'check_response_fail',
            'http_req_duration_avg_ms',
            'req_success_total',
            'req_failed_total'
        ].join(',');

        fs.writeFileSync(this.csvFile, headers + '\n');
    }

    startProcessing() {
        console.log('Iniciando processamento otimizado...');

        // Processa novas métricas a cada segundo
        this.processInterval = setInterval(() => {
            this.processNewMetricsStreaming();
        }, 1000);

        // Agrega e salva dados a cada intervalo configurado
        this.aggregateInterval = setInterval(() => {
            if (this.metricsBuffer.length > 0) {
                this.aggregateAndSave();
                this.processedIntervals++;
            }
        }, this.intervalSeconds * 1000);

        this.checkK6Status();
    }

    async processNewMetricsStreaming() {
        try {
            if (!fs.existsSync(this.rawJsonPath)) return;

            const stats = fs.statSync(this.rawJsonPath);
            if (stats.size <= this.lastFilePosition) return;

            // Cria stream apenas da parte nova do arquivo
            const stream = fs.createReadStream(this.rawJsonPath, {
                start: this.lastFilePosition,
                encoding: 'utf8'
            });

            const rl = readline.createInterface({
                input: stream,
                crlfDelay: Infinity
            });

            let newLinesCount = 0;
            let bytesRead = 0;

            rl.on('line', (line) => {
                if (line.trim()) {
                    try {
                        const data = JSON.parse(line);
                        this.processMetric(data);
                        newLinesCount++;
                    } catch (e) {
                        // Ignora linhas inválidas
                    }
                }
                bytesRead += Buffer.byteLength(line, 'utf8') + 1; // +1 para quebra de linha
            });

            rl.on('close', () => {
                this.lastFilePosition = stats.size;
                this.totalLinesProcessed += newLinesCount;
                
                if (newLinesCount > 0) {
                    console.log(`Processadas ${newLinesCount} novas linhas. Total: ${this.totalLinesProcessed} linhas`);
                }
            });

        } catch (error) {
            console.error(`Erro no streaming: ${error.message}`);
        }
    }

    processMetric(data) {
        if (data.type !== 'Point' || !data.metric || !data.data) return;

        if (!this.startTime && data.timestamp) {
            this.startTime = new Date(data.timestamp).getTime();
        }

        // Coletamos apenas as métricas relevantes
        const relevantMetrics = ['vus', 'http_req_duration', 'req_success', 'req_failed', 'checks'];
        if (relevantMetrics.includes(data.metric)) {
            this.metricsBuffer.push(data);
        }
    }

    aggregateAndSave() {
        const elapsedSeconds = this.processedIntervals * this.intervalSeconds;
        let vus = 0;
        let check2xxSuccess = 0;
        let check2xxFail = 0;
        let checkRespTimeSuccess = 0;
        let checkRespTimeFail = 0;
        let reqSuccess = 0;
        let reqFail = 0;
        const durations = [];

        this.metricsBuffer.forEach(point => {
            const { metric, data } = point;
            const value = data.value;
            const tags = data.tags || {};

            switch (metric) {
                case 'vus':
                    vus = Math.max(vus, value); // Pega o valor máximo de VUs no intervalo
                    break;
                case 'http_req_duration':
                    durations.push(value);
                    break;
                case 'req_success':
                    reqSuccess += value;
                    break;
                case 'req_failed':
                    reqFail += value;
                    break;
                case 'checks':
                    if (tags && tags.check) {
                        // Check de Status 2xx
                        if (tags.check.includes('Status 2xx')) {
                            if (value === 1) check2xxSuccess++;
                            else check2xxFail++;
                        }

                        // Check de tempo de resposta < 30s
                        if (tags.check.includes('Response time less than')) {
                            if (value === 1) checkRespTimeSuccess++;
                            else checkRespTimeFail++;
                        }
                    }
                    break;
            }
        });

        const avgDuration = durations.length > 0
            ? (durations.reduce((a, b) => a + b, 0) / durations.length)
            : 0;

        const row = [
            elapsedSeconds,
            vus,
            check2xxSuccess,
            check2xxFail,
            checkRespTimeSuccess,
            checkRespTimeFail,
            avgDuration.toFixed(2),
            reqSuccess,
            reqFail
        ].join(',');

        fs.appendFileSync(this.csvFile, row + '\n');
        
        console.log(`[${elapsedSeconds}s] VUs: ${vus}, 2xx Success: ${check2xxSuccess}, Failed: ${check2xxFail}, Avg Duration: ${avgDuration.toFixed(2)}ms`);
        
        this.metricsBuffer = [];
    }

    checkK6Status() {
        const { exec } = require('child_process');
        let consecutiveNotFound = 0;
        const maxConsecutiveNotFound = 3;

        const checkInterval = setInterval(() => {
            exec('tasklist /fi "imagename eq k6.exe" 2>nul | find /i "k6.exe" || pgrep k6 2>/dev/null',
                (error, stdout) => {
                    const k6Running = !error && (stdout.includes('k6.exe') || stdout.includes('k6'));
                    
                    if (!k6Running) {
                        consecutiveNotFound++;
                        console.log(`K6 não encontrado (${consecutiveNotFound}/${maxConsecutiveNotFound})`);
                        
                        if (consecutiveNotFound >= maxConsecutiveNotFound) {
                            console.log('K6 terminou, finalizando processamento...');
                            this.finalize(checkInterval);
                        }
                    } else {
                        consecutiveNotFound = 0; // Reset contador se K6 for encontrado
                    }
                });
        }, 3000);
    }

    async finalize(checkInterval) {
        clearInterval(this.processInterval);
        clearInterval(this.aggregateInterval);
        clearInterval(checkInterval);

        console.log('Processando métricas finais...');
        
        // Processa métricas restantes
        await this.processNewMetricsStreaming();
        
        // Salva buffer final se houver dados
        if (this.metricsBuffer.length > 0) {
            this.aggregateAndSave();
        }

        console.log(`Processamento finalizado. Total de linhas processadas: ${this.totalLinesProcessed}`);
        console.log(`Arquivo CSV salvo em: ${this.csvFile}`);
        
        process.exit(0);
    }
}

// Tratamento de sinais para finalização limpa
process.on('SIGINT', () => {
    console.log('\nFinalizando por SIGINT...');
    process.exit(0);
});

process.on('SIGTERM', () => {
    console.log('\nFinalizando por SIGTERM...');
    process.exit(0);
});

// Tratamento de erros não capturados
process.on('uncaughtException', (error) => {
    console.error('Erro não capturado:', error.message);
    process.exit(1);
});

process.on('unhandledRejection', (reason, promise) => {
    console.error('Promise rejeitada:', reason);
    process.exit(1);
});

new OptimizedMetricsProcessor();