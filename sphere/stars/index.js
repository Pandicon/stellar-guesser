const fs = require('fs');

// Read the contents of stars.csv
const data_orig = fs.readFileSync('stars.csv', 'utf-8')
  .trim()
  .split('\n')
  .filter(line => line.trim().split(",").length > 2)
  .map(line => line.trim());
const data_new = fs.readFileSync('stars-new.csv', 'utf-8')
  .split('\n')
  .filter(line => line.trim().split(",").length > 2)
  .map(line => line.trim());

let new_data = [];
for(let i = 0; i < data_orig.length; i += 1) {
	let spl_o = data_orig[i].split(",");
	let spl_n = data_new[i].split(",");
	if(Math.pow(parseFloat(spl_o[0]) - parseFloat(spl_n[0]), 2) + Math.pow(parseFloat(spl_o[1]) - parseFloat(spl_n[1]), 2) > 0.0001) {
		console.log(i);
	}
	let line = data_orig[i] + "," + spl_n[spl_n.length - 1];
	new_data.push(line);
}

// Write the trimmed data into stars-merged.csv
fs.writeFileSync('stars-merged.csv', new_data.join('\n'), 'utf-8');

console.log('File processed and saved as stars-merged.csv');