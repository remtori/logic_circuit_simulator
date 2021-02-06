const path = require('path');
const TerserPlugin = require('terser-webpack-plugin');
const HTMLWebpackPlugin = require('html-webpack-plugin');
const ExtractCssChunks = require('extract-css-chunks-webpack-plugin');

module.exports = (env) => {
	const dev = !env.production;

	const sourceDir = path.join(__dirname, './src');
	const staticDir = path.join(__dirname, './static');
	const entryFile = path.join(sourceDir, 'index.ts');
	const ejsTemplateFile = path.join(sourceDir, 'template.ejs');

	const cssPipeline = [
		{
			loader: ExtractCssChunks.loader,
		},
		{
			loader: '@teamsupercell/typings-for-css-modules-loader',
			options: {
				banner: '// auto-generated\n// Please do not change manually!',
				disableLocalsExport: true,
			},
		},
		{
			loader: 'css-loader',
			options: {
				modules: {
					exportLocalsConvention: 'camelCaseOnly',
					localIdentName: dev ? '[local]__[hash:base64:4]' : '[hash:base64:8]',
				},
			},
		},
		{
			loader: 'postcss-loader',
			options: {
				postcssOptions: {
					plugins: ['autoprefixer'],
				},
			},
		},
	];

	return {
		mode: dev ? 'development' : 'production',
		entry: entryFile,
		output: {
			filename: '[name].[contenthash:8].js',
			publicPath: '/',
		},
		resolve: {
			extensions: ['.ts', '.tsx', '.js', '.jsx'],
		},
		module: {
			rules: [
				{
					test: /\.wasm$/,
					type: 'webassembly/sync',
				},
				{
					test: /\.(t|j)sx?$/,
					exclude: /node_modules/,
					loader: 'babel-loader',
				},
				{
					test: /\.css$/,
					use: cssPipeline,
				},
				{
					test: /\.s(c|a)?ss$/,
					use: cssPipeline.concat([
						{
							loader: 'sass-loader',
							options: {
								sassOptions: {
									includePaths: [sourceDir],
								},
							},
						},
					]),
				},
			],
		},
		plugins: [
			new ExtractCssChunks({
				filename: '[name].[contenthash:8].css',
				chunkFilename: '[id].[contenthash:8].css',
			}),
			new HTMLWebpackPlugin({
				minify: true,
				inject: false,
				scriptLoading: 'defer',
				template: ejsTemplateFile,
			}),
		],
		optimization: {
			minimize: !dev,
			minimizer: [
				new TerserPlugin({
					parallel: true,
				}),
			],
			splitChunks: {
				cacheGroups: {
					vendor: {
						test: /[\\/]node_modules[\\/]/,
						name: 'vendors',
						chunks: 'all',
					},
				},
			},
		},
		experiments: {
			syncWebAssembly: true,
		},
		devServer: {
			contentBase: staticDir,
			compress: true,
			port: 11000,
		},
	};
};
