import { configDotenv } from "dotenv";
configDotenv();

import server from "./App.js";

const PORT = process.env.PORT || 3000;

server.listen(PORT, () => console.log(`Server running on port ${PORT}`));