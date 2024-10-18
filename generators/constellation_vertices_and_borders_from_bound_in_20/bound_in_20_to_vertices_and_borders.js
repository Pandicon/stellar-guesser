const fs = require('fs');

// Read the contents of bound_in_20.txt
const data = fs.readFileSync('bound_in_20.txt', 'utf-8');

// Split the data into lines, trim each line, and join them back with newlines
const with_ra_in_deg = data
  .split('\n')
  .filter(line => line.trim().split(" ").length > 2)
  .map(line => {
		const parts = line.trim().split(" ").map(part => part.trim());
		const mul_c = Math.pow(10, 6);
		let ra = Math.round(parseFloat(parts[0]) * 360.0 / 24.0 * mul_c) / mul_c;
		const new_line = `${ra},${parts[1]},${parts[2]}`;
		return new_line;
	});

// Write the trimmed data into output_vertices.csv
fs.writeFileSync('output_vertices.csv', "ra,dec,constellation\n"+with_ra_in_deg.join('\n'), 'utf-8');

console.log('File processed and saved as output_vertices.csv');

let prev_con = "";
let by_con = [];
for(const line of with_ra_in_deg) {
	let spl = line.split(",");
	let con = spl[2];
	let ra = spl[0];
	let dec = spl[1];
	if(prev_con !== con) {
		prev_con = con;
		by_con.push([]);
	}
	by_con[by_con.length - 1].push({ra, dec, con});
}
let lines = [];
for(const con of by_con) {
	for(let i = 0; i < con.length; i += 1) {
		let i_n = (i + 1) % con.length;
		let l = con[i];
		let l_n = con[i_n];
		lines.push(`${l.ra},${l.dec},${l_n.ra},${l_n.dec},87C5FFFF,1,0`);
	}
}

// Write the trimmed data into output_borders.csv
fs.writeFileSync('output_borders.csv', "ra_start,dec_start,ra_end,dec_end,colour,width,constellation\n"+lines.join('\n'), 'utf-8');

console.log('File processed and saved as output_borders.csv');