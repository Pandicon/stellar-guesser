const galactic_pole_de = (27.0 + 8.0 / 60.0) * (Math.PI / 180.0);
const galactic_pole_ra = (12.0 + 51.0 / 60.0) * 15.0 * (Math.PI / 180.0);
const step = 5;

function ra_to_dec(ra_deg) {
	let ra = ra_deg * (Math.PI / 180.0);
	return (
		-Math.atan(
			Math.cos(ra - galactic_pole_ra) / Math.tan(galactic_pole_de)
		) *
		(180.0 / Math.PI)
	);
}

let res = 'ra_start,dec_start,ra_end,dec_end,colour,width\n';
for (let ra = 0; ra < 360; ra += step) {
	const de_1 = ra_to_dec(ra);
	const de_2 = ra_to_dec(ra + step);
	res += `${ra}.0,${de_1},${(ra + step) % 360}.0,${
		de_2 % 360
	},A66BFF10,1.0\n`;
}
console.log(res);
