const path = require('path');

module.exports = {
    mode: 'production',
    //mode: 'development',
	entry: './src/index.js',
	experiments: {
		outputModule: true,
	},
	output: {
		path: path.resolve(__dirname, '../dist'),
		filename: 'main.js',
		library: {
			type: 'module',
		},
	},
};
