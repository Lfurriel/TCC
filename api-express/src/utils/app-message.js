class AppMessage extends Error {
    statusCode;
    status;
    data;

    constructor(message, statusCode, data) {
        super(message);
        this.statusCode = statusCode;
        this.status = statusCode >= 200 && statusCode <= 299 ? "success" : "error";
        this.data = data
        Object.setPrototypeOf(this, new.target.prototype);
    }
}

export default AppMessage;
