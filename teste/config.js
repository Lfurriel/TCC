export const BASE_URL = 'http://localhost:8080';

export const loadOptions = {
    stages: [
        { duration: '1m', target: 500 },
        { duration: '10m', target: 500 },
        { duration: '1m', target: 750 },
        { duration: '10m', target: 750 },
    ],
    thresholds: {
        http_req_duration: [
            'p(90)<10000',   // 90% das requisições < 10s
            'p(95)<15000',   // 95% das requisições < 15s
            'p(99)<25000'    // 99% das requisições < 25s
        ],
        http_req_failed: ['rate<0.01'],
    },
};

export const stressOptions = {
    stages: [
        { duration: '1m', target: 800 },
        { duration: '5m', target: 800 },
        { duration: '1m', target: 1000 },
        { duration: '5m', target: 1000 },
        { duration: '1m', target: 1200 },
        { duration: '5m', target: 1200 },
        { duration: '1m', target: 1400 },
        { duration: '5m', target: 1400 },
    ],
    thresholds: {
        http_req_duration: [
            'p(90)<10000',   // 90% das requisições < 10s
            'p(95)<15000',   // 95% das requisições < 15s
            'p(99)<25000'    // 99% das requisições < 25s
        ],
        http_req_failed: ['rate<0.01'],
    },
};

export const spikeOptions = {
    stages: [
        { duration: '10s', target: 100 },
        { duration: '1m', target: 100 },
        { duration: '10s', target: 2500 },
        { duration: '10m', target: 2500 },
        { duration: '10s', target: 100 },
        { duration: '1m', target: 100 },
    ],
    thresholds: {
        http_req_duration: [
            'p(90)<10000',  // 90% das requisições < 10s
            'p(95)<15000',  // 95% das requisições < 15s
            'p(99)<25000',  // 99% das requisições < 25s
        ],
        http_req_failed: ['rate<0.01'], // menos de 1% de falhas
    },
};

export const breakpointOptions = {
    stages: [
        { duration: '5m', target: 500 },
        { duration: '20m', target: 7000 },
    ],
    thresholds: {
        http_req_failed: ['rate<0.05'], // Falha se ≥5%
        http_req_duration: [
            'p(90)<10000',  // 90% das requisições < 10s
            'p(95)<15000',  // 95% das requisições < 15s
            'p(99)<25000',  // 99% das requisições < 25s
        ],
    },
};