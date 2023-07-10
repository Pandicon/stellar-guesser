const template = 'ra_start,dec_start,ra_end,dec_end,colour,width';
console.log('');
let res = `celestial-meridians.csv\n${template}\n`;
for (let ra = 0; ra < 360; ra += 5) {
	for (let de = -90.0; de < 90.0; de += 5) {
		if (de == 0) continue;
		res += `${ra}.0,${de}.0,${(ra + 5) % 360}.0,${de}.0,D9620D10,1.0\n`;
	}
}

res += `\nprime-meridian.csv\n${template}\n`;
for (let ra = 0; ra < 360; ra += 180) {
	for (let de = -90.0; de < 90.0; de += 5) {
		res += `${ra}.0,${de}.0,${ra}.0,${(de + 5) % 360}.0,D90D0D23,2.0\n`;
	}
}

res += `\ncelestial-lines-of-latitude.csv\n${template}\n`;
for (let ra = 0; ra < 360; ra += 5) {
	if (ra == 0 || ra == 180) continue;
	for (let de = -90.0; de < 90.0; de += 5) {
		res += `${ra}.0,${de}.0,${ra}.0,${(de + 5) % 360}.0,D9620D10,1.0\n`;
	}
}

res += `\ncelestial-equator.csv\n${template}\n`;
for (let ra = 0; ra < 360; ra += 5) {
	res += `${ra}.0,0.0,${(ra + 5) % 360}.0,0.0,D90D0D23,2.0\n`;
}
console.log(res);
