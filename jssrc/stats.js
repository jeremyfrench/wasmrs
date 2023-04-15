import { parse_csv } from "../pkg/index.js";
import { dataframe_to_html_table } from "../pkg/index.js";


let gobutton = document.getElementById('go');
gobutton.addEventListener('click', () => { 
let data = document.getElementById('csv').value;
let resultsDiv = document.getElementById('results');
try {	
  const csv = parse_csv(data);
  resultsDiv.innerHTML = dataframe_to_html_table(csv);	
}
catch(err) {
  resultsDiv.innerHTML = err;
}
});



