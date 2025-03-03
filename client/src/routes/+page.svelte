<!-- LOG IN -->
<script>
    import { Card, Button, Label, Input, Checkbox } from 'flowbite-svelte';
  </script>
  <div class="flex items-center justify-center h-screen">
  <Card>
    <form class="flex flex-col space-y-6" action="/">
      <h3 class="text-xl font-medium text-gray-900 dark:text-white">Log in to our platform</h3>
      <Label class="space-y-2">
        <span>Email</span>
        <Input type="email" name="email" placeholder="name@company.com" required />
      </Label>
      <Label class="space-y-2">
        <span>Your password</span>
        <Input type="password" name="password" placeholder="•••••" required />
      </Label>
      <div class="flex items-start">
        <Checkbox>Remember me</Checkbox>
        <a href="/" class="ms-auto text-sm text-blue-700 hover:underline dark:text-primary-500"> Forgot password? </a>
      </div>
      <Button href="/home" type="submit" color="blue" class="w-full bg-blue-400 ">Login to your account</Button>
      <div class="text-sm font-medium text-gray-500 dark:text-gray-300">
        Not registered? <a href="/signup" class="text-blue-700 hover:underline dark:text-primary-500"> Create account </a>
      </div>
    </form>
  </Card>
</div>
