
const { Command } = window.__TAURI__.shell;
const { invoke } = window.__TAURI__.core;
// import { Command } from '@tauri-apps/api/shell';

let greetInputEl;
let greetMsgEl;
let pythonInputEl;
let pythonOutputEl;

async function greet() {
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function runPython() {
  const pythonCode = pythonInputEl.value;
  try {
    console.log("Attempting to execute Python code:", pythonCode);
    const command = new Command('binaries/run', [pythonCode]);
    // const sidecar_command = Command.sidecar('binaries/run', ['python', '-c', pythonCode]);
    console.log("Command object:", command);
    
    console.log("Executing command...");
    const output = await command.execute();
    // const output = await sidecar_command.execute();
    console.log("Command execution result:", output);
    pythonOutputEl.textContent = output.stdout || output.stderr || 'No output';
  } catch (error) {
    console.error('Error executing Python:', error);
    console.error('Error name:', error.name);
    console.error('Error message:', error.message);
    console.error('Error stack:', error.stack);
    pythonOutputEl.textContent = `Error: ${error.toString()}`;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  pythonInputEl = document.querySelector("#python-input");
  pythonOutputEl = document.querySelector("#python-output");

  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  document.querySelector("#run-python").addEventListener("click", runPython);
});