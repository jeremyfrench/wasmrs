import benchmark from "benchmark";
import { pearson_correlation_coefficient as wasmpcc } from "./pkg/index.js";

// Define the benchmark function
function runBenchmark() {
console.log("running benchmark.");



const resultDiv = document.createElement('div');
document.body.appendChild(resultDiv);

  resultDiv.textContent = ` start..`;
  // Define the input data
  const data1 = [1,2,4,5];
  const data2 = [2,4,8,10];

  // Define the number of iterations to run
  const numIterations = 1000;

  // Record the start time
  const startTime = performance.now();

  // Run the benchmark
  for (let i = 0; i < numIterations; i++) {

  resultDiv.textContent = ` iteration ${i}.`;
 wasmpcc(data1, data2);
  }

  // Record the end time
  const endTime = performance.now();

  // Calculate the elapsed time
  const elapsed = endTime - startTime;

  // Log the result
  console.log(`Pearson correlation benchmark took ${elapsed} ms.`);

  // Display the result on the page
  resultDiv.textContent = `Pearson correlation benchmark took ${elapsed} ms.`;
}

export { runBenchmark }

