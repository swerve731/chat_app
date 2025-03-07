<!-- Sign up  -->
<script lang="ts">
    import Form from "./Form.svelte";
    import { Card, Button, Label, Input, Checkbox } from 'flowbite-svelte';
    export let form: {error?: string} | undefined;
    let errorMessage;

    async function handleSubmit(event: SubmitEvent) {
      console.log('submit');
      event.preventDefault();
      const formData = new FormData(event.target as HTMLFormElement);

      const response = await fetch('127.0.0.0:3000/auth/signup', {
        method: 'POST',
        body: formData
      });
      const jwt: string | null = response.headers.get('Authorization');

      
      const result = await response.json();

      if (!response.ok) {
        console.log(result.error || 'An error occurred');

      }else if (jwt){
       console.log(jwt);
       
      }
    }
    
  </script>
  <div class="flex items-center justify-center h-screen">
  <Card>
    <Form class="flex flex-col space-y-6" method="POST" on:submit={handleSubmit}>
      <h3 class="text-xl font-medium text-gray-900 dark:text-white">Sign up for our platform</h3>
      <Label class="space-y-2">
        <span>Email</span>
        <Input type="email" name="email" placeholder="name@company.com" required />
      </Label>
      <Label class="space-y-2">
        <span>Your password</span>
        <Input type="password" name="password" placeholder="•••••" required />
      </Label>
      <Label class="space-y-2">
        <span>Re-enter Password</span>
        <Input type="password" name="password" placeholder="•••••" required />
      </Label>
      <div class="flex items-start">
        <Checkbox>Remember me</Checkbox>
      </div>
      <Button href="/home"type="submit" color="blue" class="w-full bg-blue-400 ">Create to your account</Button>
    </Form>
  </Card>
</div>

