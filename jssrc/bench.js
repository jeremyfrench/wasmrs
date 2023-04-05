import _ from 'lodash';
import process from 'process';

const benchmark = require('benchmark');
const Benchmark = benchmark.runInContext({ _, process });
window.Benchmark = Benchmark;

import { pearson_correlation_coefficient as wasmpcc } from "../pkg/index.js";
import { pearsonCorrelationCoefficient as jspcc } from "./pcc.js"

// Define the benchmark function
function runBenchmark() {
console.log("running benchmark.");



const resultDiv = document.createElement('div');
document.body.appendChild(resultDiv);
var x = [];
var y = [];

for (var i=1;i<10000;i++) {
 x[i] = Math.random();
 y[i] = Math.random();
}

benchmark.options.minSamples = 10;
const suite = new benchmark.Suite();
resultDiv.textContent = "here we go!";
suite
  .add('JavaScript', function() {
   jspcc(x, y);
  })
  .add('Rust (WebAssembly)', function() {
   wasmpcc(x, y);
  })
 .on("cycle", event =>  {
         console.log(String(event.target));
	     })
.on('complete', event => {
        const suite = event.currentTarget;
	window.suit = suite;
	const fastest = 'Fastest is ' + suite.filter('fastest').map('name');
console.log(fastest);
 resultDiv.textContent = fastest; 
}).run()

}

export { runBenchmark }

