<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { fly, fade, scale } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';

	type ChatMessage = {
		id: string;
		text: string;
		user: string;
		timestamp: Date;
		self?: boolean;
	};

	let username = $state('');
	let connected = $state(false);
	let socket = $state<WebSocket | null>(null);

	let input = $state('');
	let messages = $state<ChatMessage[]>([]);

	let loading = $state(false);
	let connectionState = $state('Disconnected');

	let chatContainer = $state<HTMLDivElement | null>(null);

	function generateId() {
		return crypto.randomUUID();
	}

	async function connect() {
		if (!username.trim()) return;

		loading = true;
		connectionState = 'Connecting...';

		socket?.close();

		socket = new WebSocket(
			`ws://localhost:3000/ws/subscribe/${encodeURIComponent(username)}`
		);

		socket.onopen = () => {
			connected = true;
			loading = false;
			connectionState = 'Connected';

			messages = [
				...messages,
				{
					id: generateId(),
					user: 'system',
					text: `Connected as ${username}`,
					timestamp: new Date()
				}
			];
		};

		socket.onmessage = async (event) => {
			messages = [
				...messages,
				{
					id: generateId(),
					user: 'server',
					text: event.data,
					timestamp: new Date()
				}
			];

			await scrollToBottom();
		};

		socket.onclose = () => {
			connected = false;
			connectionState = 'Disconnected';
		};

		socket.onerror = () => {
			connected = false;
			connectionState = 'Connection failed';
			loading = false;
		};
	}

	async function sendMessage() {
		if (!input.trim()) return;

		const msg = input;

		messages = [
			...messages,
			{
				id: generateId(),
				user: username,
				text: msg,
				timestamp: new Date(),
				self: true
			}
		];

		input = '';

		await fetch(`http://localhost:3000/notify/${encodeURIComponent(msg)}`, {
			method: 'POST',
			headers: {
				'x-user': username
			}
		});

		await scrollToBottom();
	}

	async function scrollToBottom() {
		await tick();

		if (chatContainer) {
			chatContainer.scrollTo({
				top: chatContainer.scrollHeight,
				behavior: 'smooth'
			});
		}
	}

	onMount(() => {
		return () => {
			socket?.close();
		};
	});
</script>

<svelte:head>
	<title>Realtime Chat</title>
</svelte:head>

<div
	class="relative min-h-screen overflow-hidden bg-[#09090f] text-zinc-100"
>
	<!-- Ambient Background -->
	<div class="absolute inset-0 overflow-hidden">
		<div
			class="absolute -top-40 left-[-10rem] h-[30rem] w-[30rem] rounded-full bg-fuchsia-500/10 blur-3xl"
		></div>

		<div
			class="absolute bottom-[-10rem] right-[-10rem] h-[35rem] w-[35rem] rounded-full bg-cyan-500/10 blur-3xl"
		></div>

		<div
			class="absolute left-[40%] top-[20%] h-[20rem] w-[20rem] rounded-full bg-violet-500/10 blur-3xl"
		></div>
	</div>

	<div
		class="relative z-10 flex min-h-screen items-center justify-center p-6"
	>
		<div
			class="w-full max-w-5xl overflow-hidden rounded-[2rem] border border-white/10 bg-white/5 shadow-2xl backdrop-blur-xl"
			in:scale={{ duration: 500 }}
		>
			<div class="grid min-h-[780px] grid-cols-1 lg:grid-cols-[320px_1fr]">
				<!-- Sidebar -->
				<div
					class="relative border-b border-white/10 bg-gradient-to-b from-zinc-900/80 to-black/40 p-8 lg:border-b-0 lg:border-r"
				>
					<div class="space-y-8">
						<div>
							<div
								class="mb-3 inline-flex items-center gap-3 rounded-full border border-white/10 bg-white/5 px-4 py-2"
							>
								<div
									class={`h-2.5 w-2.5 rounded-full transition-all duration-500 ${
										connected
											? 'bg-emerald-400 shadow-[0_0_15px_#4ade80]'
											: 'bg-zinc-500'
									}`}
								></div>

								<span class="text-sm text-zinc-300">
									{connectionState}
								</span>
							</div>

							<h1
								class="bg-gradient-to-r from-fuchsia-300 via-violet-300 to-cyan-300 bg-clip-text text-4xl font-black tracking-tight text-transparent"
							>
								Nebula Chat
							</h1>

							<p class="mt-3 text-sm leading-relaxed text-zinc-400">
								A smooth realtime messaging experience powered by
								Svelte 5 and websockets.
							</p>
						</div>

						<div class="space-y-4">
							<label for="username" class="block text-sm text-zinc-400">
								Username
							</label>

							<input
								id="username"
								bind:value={username}
								placeholder="Choose a username..."
								class="w-full rounded-2xl border border-white/10 bg-black/30 px-5 py-4 text-zinc-100 outline-none transition-all duration-300 placeholder:text-zinc-500 focus:border-fuchsia-500/40 focus:bg-black/50"
							/>

							<button
								onclick={connect}
								disabled={loading || connected}
								class="group relative w-full overflow-hidden rounded-2xl border border-fuchsia-400/20 bg-gradient-to-r from-fuchsia-600 to-violet-600 px-5 py-4 font-medium text-white transition-all duration-300 hover:scale-[1.02] hover:shadow-[0_0_30px_rgba(168,85,247,0.35)] disabled:cursor-not-allowed disabled:opacity-50"
							>
								<div
									class="absolute inset-0 bg-white/10 opacity-0 transition-opacity duration-300 group-hover:opacity-100"
								></div>

								<span class="relative">
									{#if loading}
										Connecting...
									{:else if connected}
										Connected
									{:else}
										Connect
									{/if}
								</span>
							</button>
						</div>

						<div
							class="rounded-3xl border border-white/10 bg-black/20 p-5"
						>
							<h2 class="mb-3 text-sm font-semibold text-zinc-300">
								Features
							</h2>

							<div class="space-y-3 text-sm text-zinc-400">
								<div class="flex items-center gap-3">
									<div
										class="h-2 w-2 rounded-full bg-cyan-400"
									></div>
									WebSocket realtime updates
								</div>

								<div class="flex items-center gap-3">
									<div
										class="h-2 w-2 rounded-full bg-fuchsia-400"
									></div>
									Smooth animated UI
								</div>

								<div class="flex items-center gap-3">
									<div
										class="h-2 w-2 rounded-full bg-violet-400"
									></div>
									Dark glassmorphism theme
								</div>
							</div>
						</div>
					</div>
				</div>

				<!-- Chat -->
				<div class="flex flex-col">
					<!-- Header -->
					<div
						class="flex items-center justify-between border-b border-white/10 bg-black/20 px-6 py-5 backdrop-blur-xl"
					>
						<div>
							<h2 class="text-xl font-semibold">
								Realtime Messages
							</h2>

							<p class="mt-1 text-sm text-zinc-400">
								Messages appear instantly across connected users.
							</p>
						</div>

						<div
							class="rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-zinc-300"
						>
							{messages.length} messages
						</div>
					</div>

					<!-- Messages -->
					<div
						bind:this={chatContainer}
						class="flex-1 space-y-4 overflow-y-auto px-6 py-6"
					>
						{#if messages.length === 0}
							<div
								class="flex h-full items-center justify-center"
								in:fade
							>
								<div class="text-center">
									<div
										class="mb-5 text-7xl opacity-40"
									>
										✦
									</div>

									<h3
										class="text-xl font-semibold text-zinc-300"
									>
										No messages yet
									</h3>

									<p class="mt-2 text-zinc-500">
										Connect and start chatting.
									</p>
								</div>
							</div>
						{/if}

						{#each messages as message (message.id)}
							<div
								class={`flex ${
									message.self
										? 'justify-end'
										: 'justify-start'
								}`}
								in:fly={{
									y: 20,
									duration: 350,
									easing: cubicOut
								}}
							>
								<div
									class={`max-w-[75%] rounded-3xl border px-5 py-4 shadow-lg backdrop-blur-xl transition-all duration-300 ${
										message.self
											? 'border-fuchsia-500/20 bg-gradient-to-br from-fuchsia-500/20 to-violet-500/10'
											: 'border-white/10 bg-white/5'
									}`}
								>
									<div
										class="mb-2 flex items-center gap-2 text-xs"
									>
										<span
											class={`font-semibold ${
												message.self
													? 'text-fuchsia-300'
													: 'text-cyan-300'
											}`}
										>
											{message.user}
										</span>

										<span class="text-zinc-500">
											{message.timestamp.toLocaleTimeString()}
										</span>
									</div>

									<p
										class="leading-relaxed text-zinc-100"
									>
										{message.text}
									</p>
								</div>
							</div>
						{/each}
					</div>

					<!-- Input -->
					<div
						class="border-t border-white/10 bg-black/20 p-5 backdrop-blur-xl"
					>
						<div class="flex gap-4">
							<input
								bind:value={input}
								onkeydown={(e) =>
									e.key === 'Enter' && sendMessage()}
								placeholder="Write a message..."
								disabled={!connected}
								class="flex-1 rounded-2xl border border-white/10 bg-white/5 px-5 py-4 text-zinc-100 outline-none transition-all duration-300 placeholder:text-zinc-500 focus:border-cyan-500/30 focus:bg-white/[0.08] disabled:opacity-50"
							/>

							<button
								onclick={sendMessage}
								disabled={!connected}
								class="rounded-2xl border border-cyan-400/20 bg-gradient-to-r from-cyan-600 to-blue-600 px-7 py-4 font-medium text-white transition-all duration-300 hover:scale-[1.03] hover:shadow-[0_0_25px_rgba(34,211,238,0.35)] disabled:cursor-not-allowed disabled:opacity-50"
							>
								Send
							</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
