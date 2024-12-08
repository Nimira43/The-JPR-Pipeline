const fs = require('fs');
const seedrandom = require('seedrandom');

const rng = seedrandom('my-seed');

const data = [];
for (let i = 0; i < 100; i++) {
  const x = rng() * 100;
  const y = rng() * 100;
  data.push({ x, y });
}

fs.writeFileSync('data.json', JSON.stringify(data, null, 2));

console.log('Data generated and saved to data.json');
