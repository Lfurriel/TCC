package utils

type AppMessage struct {
	StatusCode int         `json:"statusCode"`
	Status     string      `json:"status"`
	Message    string      `json:"message"`
	Data       interface{} `json:"data,omitempty"`
	Errors     interface{} `json:"errors,omitempty"`
}

func NewAppMessage(message string, statusCode int, data interface{}, errors ...interface{}) *AppMessage {
	status := "error"
	if statusCode >= 200 && statusCode <= 299 {
		status = "success"
	}

	var errData interface{}
	if len(errors) > 0 {
		errData = errors[0]
	}

	return &AppMessage{
		StatusCode: statusCode,
		Status:     status,
		Message:    message,
		Data:       data,
		Errors:     errData,
	}
}
