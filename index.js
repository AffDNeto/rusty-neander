// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
require('./neander.js');
const rust = import('./pkg');

rust
  .then(m => { m.greet('World!'); nn = m } )
  .catch(console.error);