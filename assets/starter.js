//const elStarter = document.querySelector('#el-starter')

//elStarter.addEventListener('click', run, {
	//once: true,
	//passive: true,
//})

async function run() {
	//elStarter.removeEventListener('click', run )
	//elStarter.remove()
	const main = import('./domik.js').then(
		({default: init, main}) =>
		init().then(
				() => main
			)
	)
	//const f = await main;
	//f();
}

run();



console.log('--> activated starter.js')
