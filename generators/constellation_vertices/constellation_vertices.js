const fetch = require('node-fetch');

const constellations = [
	'And',
	'Ant',
	'Aps',
	'Aqr',
	'Aql',
	'Ara',
	'Ari',
	'Aur',
	'Boo',
	'Cae',
	'Cam',
	'Cnc',
	'CVn',
	'CMa',
	'CMi',
	'Cap',
	'Car',
	'Cas',
	'Cen',
	'Cep',
	'Cet',
	'Cha',
	'Cir',
	'Col',
	'Com',
	'CrA',
	'CrB',
	'Crv',
	'Crt',
	'Cru',
	'Cyg',
	'Del',
	'Dor',
	'Dra',
	'Equ',
	'Eri',
	'For',
	'Gem',
	'Gru',
	'Her',
	'Hor',
	'Hya',
	'Hyi',
	'Ind',
	'Lac',
	'Leo',
	'LMi',
	'Lep',
	'Lib',
	'Lup',
	'Lyn',
	'Lyr',
	'Men',
	'Mic',
	'Mon',
	'Mus',
	'Nor',
	'Oct',
	'Oph',
	'Ori',
	'Pav',
	'Peg',
	'Per',
	'Phe',
	'Pic',
	'Psc',
	'PsA',
	'Pup',
	'Pyx',
	'Ret',
	'Sge',
	'Sgr',
	'Sco',
	'Scl',
	'Sct',
	'Ser',
	'Sex',
	'Tau',
	'Tel',
	'Tri',
	'TrA',
	'Tuc',
	'UMa',
	'UMi',
	'Vel',
	'Vir',
	'Vol',
	'Vul'
];

async function run() {
	for (const constellation of constellations) {
		const response = await fetch(
			`https://www.iau.org/static/public/constellations/txt/${constellation.toLowerCase()}.txt`
		);
		const data = await response.text();
		for (const line of data.split('\n')) {
			let line_split = line.split('|').map((el) => el.trim());
			if (line_split.length < 3) continue;
			let ra_hours = line_split[0].split(' ').map((el) => parseFloat(el));
			let ra = (ra_hours[0] + ra_hours[1] / 60 + ra_hours[2] / 3600) * 15;
			console.log(
				`${ra},${line_split[1]},${line_split[2].toUpperCase()}`
			);
		}
	}
}

run();
