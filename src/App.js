import express from "express";
import cors from "cors";

import { JsonSyntaxError } from "./helpers/error";

class App {
    constructor() {
        this.server = express();
        this.middlewares();
    }

    middlewares() {
        this.server.use(cors());
        this.server.use(express.json());
        this.server.use(JsonSyntaxError);
    }
}

export default new App().server;