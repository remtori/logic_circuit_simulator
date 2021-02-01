import { h } from 'preact';
import { useEffect, useRef } from 'preact/hooks';
import $css from './index.scss';

export const CircuitList = () => (
	<div class={$css.circuitList}>
		<div class={$css.circuitItem}>
			<span>AND</span>
		</div>
		<div class={$css.circuitItem}>
			<span>NOT</span>
		</div>
		<div class={$css.circuitItem}>
			<span>NAND</span>
		</div>
		<div class={$css.circuitItem}>
			<span>OR</span>
		</div>
		<div class={$css.circuitItem}>
			<span>AND</span>
		</div>
		<div class={$css.circuitItem}>
			<span>NOT</span>
		</div>
		<div class={$css.circuitItem}>
			<span>NAND</span>
		</div>
		<div class={$css.circuitItem}>
			<span>OR</span>
		</div>
		<div class={$css.circuitItem}>
			<span>AND</span>
		</div>
		<div class={$css.circuitItem}>
			<span>NOT</span>
		</div>
		<div class={$css.circuitItem}>
			<span>NAND</span>
		</div>

		<div class={$css.circuitItem}>
			<span>OR</span>
		</div>
		<div class={$css.circuitItem}>
			<span>AND</span>
		</div>
		<div class={$css.circuitItem}>
			<span>NOT</span>
		</div>
		<div class={$css.circuitItem}>
			<span>NAND</span>
		</div>
	</div>
);

export const App = () => {
	const canvasRef = useRef<HTMLCanvasElement>();

	useEffect(() => {
		if (!canvasRef.current) return;

		const { width, height } = canvasRef.current;
		const context = canvasRef.current.getContext('2d')!;

		context.fillStyle = '#000';
		context.fillRect(0, 0, width, height);
	}, [canvasRef.current]);

	return (
		<div id="app" class={$css.app}>
			<canvas ref={canvasRef} width={640} height={360} />
			<CircuitList />
		</div>
	);
};
