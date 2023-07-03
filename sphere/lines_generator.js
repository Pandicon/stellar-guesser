let res = '';
for (let i = 0; i < 360; i += 5) {
	for (let j = -90.0; j < 90.0; j += 5) {
		let width = j == 0 ? 2 : 1;
		let colour = j == 0 ? 'D90D0D23' : 'D9620D10';
		res += `${i}.0,${j}.0,${i + 5}.0,${j}.0,${colour},${width}.0\n`;
	}
}
for (let i = 0; i < 360; i += 5) {
	for (let j = -90.0; j < 90.0; j += 5) {
		let width = i == 0 || i == 180 ? 2 : 1;
		let colour = i == 0 || i == 180 ? 'D90D0D23' : 'D9620D10';
		res += `${i}.0,${j}.0,${i}.0,${j + 5}.0,${colour},${width}.0\n`;
	}
}
console.log(res);
