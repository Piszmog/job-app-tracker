module.exports = {
	plugins: {
		tailwindcss: {},
		autoprefixer: {},
		...(process.env.TAURI_DEBUG === 'production' ? { cssnano: {} } : {})
	}
};
