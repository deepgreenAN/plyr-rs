const path = require('path');

module.exports = {
    // mode: 'production',
    mode: 'development',
	entry: './src/index.js',
	output: {
		path: path.resolve(__dirname, '../dist'),
		filename: 'main.js'
	},
	// devServer: {
	// 	static: './../js_dist'
	// },
    // devtool: 'inline-source-map',
};
