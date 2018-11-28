# How to debug Rust apps?

## VSCode

`lldb` 3.8 is problematic. use `rust-gdb` instead.

`.vscode/launch.json`

```
{
	// Use IntelliSense to learn about possible attributes.
	// Hover to view descriptions of existing attributes.
	// For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
	"version": "0.2.0",
		"configurations": [
		{
			"type": "gdb",
			"request": "launch",
			"gdbpath": "rust-gdb",
			"name": "Debug rustup-init",
			"target": "target/debug/rustup-init",
			"cwd": "${workspaceRoot}",
			"arguments": "target add x86_64-unknown-linux-gnu"
		},
		]
}
```

