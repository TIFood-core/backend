export function JsonSyntaxError(err, req, res, next) {
    if (err instanceof SystaxError && err.status === 400 && "body" in err) {
        return res.status(422).json({"error": "Invalid JSON"});
    }

    next();
}