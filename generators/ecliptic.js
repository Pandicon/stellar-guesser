const ecliptic_inclination = 23 + 26.3 / 60.0;
const step = 5;

let res = 'ra_start,dec_start,ra_end,dec_end,colour,width\n';
for (let ra = 0; ra < 360; ra += step) {
	const de_1 =
		(Math.atan(
			Math.sin((ra * Math.PI) / 180.0) *
				Math.tan((ecliptic_inclination * Math.PI) / 180.0)
		) *
			180.0) /
		Math.PI;
	const de_2 =
		(Math.atan(
			Math.sin(((ra + step) * Math.PI) / 180.0) *
				Math.tan((ecliptic_inclination * Math.PI) / 180.0)
		) *
			180.0) /
		Math.PI;
	res += `${ra}.0,${de_1},${(ra + step) % 360}.0,${
		de_2 % 360
	},6BFF6B10,1.0\n`;
}
console.log(res);
