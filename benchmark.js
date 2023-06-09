import _ from 'lodash';
import process from 'process';

const benchmark = require('benchmark');
const Benchmark = benchmark.runInContext({ _, process });
window.Benchmark = Benchmark;

import { pearson_correlation_coefficient as wasmpcc } from "./pkg/index.js";
import { pearsonCorrelationCoefficient as jspcc } from "./pcc.js"

// Define the benchmark function
function runBenchmark() {
console.log("running benchmark.");



const resultDiv = document.createElement('div');
document.body.appendChild(resultDiv);
const x = [1.0, 2.0, 3.0, 4.0, 5.0];
const y = [2.0, 4.0, 6.0, 8.0, 10.0];

const suite = new benchmark.Suite();
resultDiv.textContent = "here we go!";
suite
  .add('JavaScript', function() {
   jspcc(x, y);
  })
  .add('Rust (WebAssembly)', function() {
   wasmpcc(x, y);
  })
.on('complete', event => {
        const suite = event.currentTarget;
 resultDiv.textContent = suite.toString(); 
}).run()

}

export { runBenchmark }

