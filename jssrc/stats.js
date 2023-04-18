import { csv_to_analysis } from "../pkg/index.js";


let gobutton = document.getElementById('go');
gobutton.addEventListener('click', () => { 
let data = document.getElementById('csv').value;
let resultsDiv = document.getElementById('results');
try {	
  resultsDiv.innerHTML = csv_to_analysis(data);	
}
catch(err) {
  resultsDiv.innerHTML = err;
}
});



