import init, { run_app } from './pkg/front_end_training_project.js';
async function main() {
   await init('/pkg/front_end_training_project_bg.wasm');
   run_app();
}
main()