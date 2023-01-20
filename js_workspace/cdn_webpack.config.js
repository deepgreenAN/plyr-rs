const path = require('path');

module.exports = {
    // mode: 'production',
    mode: 'development',
	entry: './src/index.js',
	output: {
		path: path.resolve(__dirname, '../dist_cdn'),
		filename: 'main.js'
	},
    externals: {
        plyr: "plyr"
    }
	// devServer: {
	// 	static: './../js_dist'
	// },
    // devtool: 'inline-source-map',
};
