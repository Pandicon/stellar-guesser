const fs = require('fs');

// Read the contents of bound_in_20.txt
const data_1 = fs.readFileSync('constellation_vertices.csv', 'utf-8');
const data_2 = fs.readFileSync('constellations_old.csv', 'utf-8');

let d = data_1
  .split('\n')
  .filter(line => line.trim().split(",").length > 2);
let d_2 = data_2
  .split('\n')
  .filter(line => line.trim().split(",").length > 1);
let prev_con = "";
let l = "";
let i = -1;
for(const line of d) {
	let spl = line.split(",");
	let con = spl[2];
	let ra = spl[0];
	let dec = spl[1];
	if(prev_con !== con) {
		prev_con = con;
		if(con === "SER2") {
			l = l.slice(0, -1);
			l += "#";
		} else {
			// console.log(prev_con + ": " + l);
			console.log(l.slice(0, -1));
			i += 1;
			l = d_2[i].trim() + ",";
		}
	}
	l += `${ra};${dec}|`
}
console.log(l.slice(0, -1));