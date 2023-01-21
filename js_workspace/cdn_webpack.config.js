const path = require('path');

module.exports = {
    mode: 'production',
    //mode: 'development',
	entry: './src/index.js',
	experiments: {
		outputModule: true,
	},
	output: {
		path: path.resolve(__dirname, '../dist_cdn'),
		filename: 'main.js',
		library: {
			type: 'module',
		},
	},
	externalsType: 'global',
	// externalsType: 'window', // こちらでもよい
    externals: {
        plyr: "Plyr"
    }
};
