#!/usr/bin/env node
import { execFileSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const protoDir = path.resolve(fileURLToPath(new URL('../../protocols', import.meta.url)));
const outDir = path.resolve(fileURLToPath(new URL('../src/lib/proto', import.meta.url)));

// recursively collect all .proto files
function getProtoFiles(dir) {
	const files = [];
	for (const f of fs.readdirSync(dir, { withFileTypes: true })) {
		const fp = path.join(dir, f.name);
		if (f.isDirectory()) {
			files.push(...getProtoFiles(fp));
		} else if (f.name.endsWith('.proto')) {
			files.push(fp);
		}
	}
	return files;
}

// --- COLLECT PROTO FILES ---
const protoFiles = getProtoFiles(protoDir);
if (protoFiles.length === 0) {
	console.error('No .proto files found in', protoDir);
	process.exit(1);
}

console.log('[proto] Generating TypeScript from .proto files:');
protoFiles.forEach((f) => console.log('  ', f));

// --- CLEAN THE OUTPUT FOLDER ---
if (fs.existsSync(outDir)) {
	console.log('[proto] Cleaning output folder', outDir);
	fs.rmSync(outDir, { recursive: true, force: true });
}
fs.mkdirSync(outDir, { recursive: true });

// --- RUN PROTOC ---
execFileSync(
	'protoc',
	[
		'-I',
		protoDir,
		...protoFiles,
		'--plugin=./node_modules/.bin/protoc-gen-ts_proto',
		`--ts_proto_out=${outDir}`,
		'--ts_proto_opt=esModuleInterop=true,outputJsonMethods=true,forceLong=string'
	],
	{ stdio: 'inherit' }
);

console.log('[proto] Generation complete');
