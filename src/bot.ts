import { Parachute, Permission } from "./parachute";
import * as fs from "fs-extra";
import { Client, Message, Collection, Member } from "eris";

const keys = JSON.parse(fs.readFileSync("./confs/keys.json", "utf8"));
const settings = JSON.parse(fs.readFileSync("./confs/settings.json", "utf8"));

const token = keys["token"];
const owner = settings["owner"];
const prefix = settings["command_prefix"];

const parachute = new Parachute(token, owner, prefix);

fs.readdir("./src/plugins/", (err: NodeJS.ErrnoException, files: string[]) => {
	files.forEach((file: string) => {
		const m = file.match(/([a-z0-9_]+)\..{1,4}$/);
		if (m) {
			import("./plugins/" + m[1]).then(plugin => {
				parachute.register_command(plugin.default);
			});
		}
	});
});

export = parachute;
