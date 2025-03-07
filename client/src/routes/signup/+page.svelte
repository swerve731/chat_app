<!-- src/routes/signup/+page.svelte (or wherever you place this) -->
<script lang="ts">
	import { Card, Button, Label, Input, Checkbox } from 'flowbite-svelte';
	import { goto } from '$app/navigation';

	let errorMessage: string = '';
	let successMessage: string = '';

	async function signup(event: SubmitEvent): Promise<void> {
		event.preventDefault();
		const form = event.target as HTMLFormElement;
		const email = form.email.value; // Assuming 'email' is meant to be 'email'
		const password = form.password.value;
		const confirmPassword = form.confirmPassword.value;

		// Client-side validation
		if (password !== confirmPassword) {
			errorMessage = 'Passwords do not match';
			return;
		}

		const params = new URLSearchParams();
		params.append('email', email);
		params.append('password', password);

		try {
			const response = await fetch('http://localhost:3000/auth/signup', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/x-www-form-urlencoded'
				},
				body: params
			});

			if (!response.ok) {
				const errorText: string = await response.text();
				errorMessage = errorText || 'Failed to create account';
				successMessage = '';
				console.error('Error:', errorText);
				return;
			}

			console.log('All headers:', Object.fromEntries(response.headers.entries()));
			const jwt: string | null = response.headers.get('Authorization');
			const responseText: string = await response.text();
			if (jwt) {
				localStorage.setItem('token', jwt);

				// this is how you will get the jwt
				// let jwt = localStorage.getItem("token")

				successMessage = responseText || 'Account successfully created';
				errorMessage = '';

				await goto('/home');
			} else {
				errorMessage = 'No JWT returned in Authorization header';
				successMessage = '';
				console.warn('No JWT returned');
			}
		} catch (error) {
			errorMessage = 'Network error: ' + (error as Error).message;
			successMessage = '';
			console.error('Network error:', error);
		}
	}
</script>

<div class="flex h-screen items-center justify-center">
	<Card>
		<form class="flex flex-col space-y-6" method="POST" on:submit={signup}>
			<h3 class="text-xl font-medium text-gray-900 dark:text-white">Sign up for our platform</h3>

			<Label class="space-y-2">
				<span>email</span>
				<Input type="text" name="email" placeholder="email" required />
			</Label>

			<Label class="space-y-2">
				<span>Your password</span>
				<Input type="password" name="password" placeholder="•••••" required />
			</Label>

			<Label class="space-y-2">
				<span>Re-enter Password</span>
				<Input type="password" name="confirmPassword" placeholder="•••••" required />
			</Label>

			<div class="flex items-start">
				<Checkbox>Remember me</Checkbox>
			</div>

			{#if errorMessage}
				<p class="text-red-500">{errorMessage}</p>
			{/if}
			{#if successMessage}
				<p class="text-green-500">{successMessage}</p>
			{/if}

			<Button type="submit" color="blue" class="w-full bg-blue-400">Create your account</Button>
		</form>
	</Card>
</div>
