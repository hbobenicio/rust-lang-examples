const wasmLoader = import('./wasm_greet');

wasmLoader.then(Greet => {
  console.log('WebAssembly module loaded successfully:', Greet);

  Greet.greet('World!');

  const total = Greet.add(10, 20);
  console.log(`Total: ${total}`);

  console.log('Person:', Greet.Person);
  const p = new Greet.Person('Foo', 20);
  console.log(`name: ${p.getName()} / age: ${p.age}`);
  p.birthday();
  console.log(`name: ${p.getName()} / age: ${p.age}`);
})
.catch(error => {
  console.error('Failed to load was module "wasm_greet".');
});
