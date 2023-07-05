function deg_to_rad(deg) {
	return (deg / 180.0) * Math.PI;
}

function convert_dec(dec, t) {
	let dec_2000 = dec + 0.0416667 + 1.385 * Math.pow(10, -7) * t;

	let l =
		280.46 +
		36000.771 * t +
		0.0003875 * Math.pow(t, 2) -
		Math.pow(t, 3) / 38710000;
	let s =
		125.04452 -
		1934.136261 * t +
		0.0020708 * Math.pow(t, 2) +
		Math.pow(t, 3) / 450000;

	let d_dec_p =
		-0.0252 * t + 0.00929 * Math.pow(t, 2) + 0.00006 * Math.pow(t, 3);
	let d_dec_n =
		9.2 * Math.cos(deg_to_rad(s)) +
		0.57 * Math.cos(deg_to_rad(2 * l)) +
		0.1 * Math.cos(deg_to_rad(2 * l - s)) -
		0.09 * Math.cos(deg_to_rad(2 * s));

	return dec_2000 + d_dec_p + d_dec_n;
}

function convert_ra(ra, t) {
	let ra_2000 = ra + 0.0083333 + 1.397 * Math.pow(10, -7) * t;

	let l =
		280.46 +
		36000.771 * t +
		0.0003875 * Math.pow(t, 2) -
		Math.pow(t, 3) / 38710000;
	let s =
		125.04452 -
		1934.136261 * t +
		0.0020708 * Math.pow(t, 2) +
		Math.pow(t, 3) / 450000;

	let d_ra_p =
		-47.0029 * t - 0.06603 * Math.pow(t, 2) - 0.00006 * Math.pow(t, 3);
	let d_ra_n =
		-17.2 * Math.sin(deg_to_rad(s)) -
		1.32 * Math.sin(deg_to_rad(2 * l)) -
		0.23 * Math.sin(deg_to_rad(2 * l - s)) +
		0.21 * Math.sin(deg_to_rad(2 * s));

	return ra_2000 + d_ra_p + d_ra_n;
}

function flip_dec_over_poles(dec) {
	if (dec > 90) {
		return 180 - dec;
	} else if (dec < -90) {
		return -180 - dec;
	} else {
		return dec;
	}
}

const step = 10; // In degrees

const data = ``; // constellation_borders_2000_raw.txt

const split_data = data.split('\n');
const grouped_data = [];
let c = '';
for (let i = 0; i < split_data.length; i += 1) {
	let s = split_data[i].split(' ');
	if (c != s[2]) {
		c = s[2];
		grouped_data.push([]);
	}
	grouped_data[grouped_data.length - 1].push([s[2], s[0], s[1]]);
}
const new_data = [];
for (let i = 0; i < grouped_data.length; i += 1) {
	new_data.push([]);
	for (let j = 0; j < grouped_data[i].length - 1; j += 1) {
		new_data[i].push([
			parseFloat(grouped_data[i][j][1]) * 15,
			grouped_data[i][j][2],
			parseFloat(grouped_data[i][(j + 1) % grouped_data[i].length][1]) *
				15,
			grouped_data[i][(j + 1) % grouped_data[i].length][2],
			'87C5FF3F',
			1.0,
			grouped_data[i][j][0]
		]);
	}
}
/*for (const constellation of grouped_data) {
	new_data.push([]);
	for (const line of constellation) {
		let ra_s = parseFloat(line[0]);
		let de_s = parseFloat(line[1]);
		let ra_e = parseFloat(line[2]);
		let de_e = parseFloat(line[3]);
		if (ra_s > ra_e) {
			let h = ra_e;
			ra_e = ra_s;
			ra_s = h;
		}
		if (de_s > de_e) {
			let h = de_e;
			de_e = de_s;
			de_s = h;
		}
		if (ra_e - ra_s > 120) {
			ra_s += 360;
		}
		if (de_e - de_s > 120) {
			ra_s += 360;
		}
		if (ra_s > ra_e) {
			let h = ra_e;
			ra_e = ra_s;
			ra_s = h;
		}
		if (de_s > de_e) {
			let h = de_e;
			de_e = de_s;
			de_s = h;
		}
		let d_ra = ra_e - ra_s;
		let d_de = de_e - de_s;
		let ra_steps = Math.ceil(d_ra / step);
		let de_steps = Math.ceil(d_de / step);
		let steps = Math.max(ra_steps, de_steps);
		let ra_step = d_ra / steps;
		let de_step = d_de / steps;
		for (let j = 0; j < steps; j += 1) {
			let ra_1 = convert_ra((ra_s + ra_step * j) % 360, 0.23) % 360;
			let dec_1 = flip_dec_over_poles(
				convert_dec((de_s + de_step * j) % 360, 0.23)
			);
			let ra_2 = convert_ra((ra_s + ra_step * (j + 1)) % 360, 0.23) % 360;
			let dec_2 = flip_dec_over_poles(
				convert_dec((de_s + de_step * (j + 1)) % 360, 0.23)
			);
			new_data[new_data.length - 1].push([
				ra_1.toString(10),
				dec_1.toString(10),
				ra_2.toString(10),
				dec_2.toString(10),
				line[4],
				line[5],
				line[6]
			]);
		}
	}
}*/
const joined_lines = [];
for (const constellation of new_data) {
	for (const line of constellation) {
		joined_lines.push(line.join(','));
	}
}
console.log(joined_lines.join('\n'));
