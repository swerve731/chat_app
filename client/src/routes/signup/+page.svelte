<!-- Sign up  -->
<script>
    import { Card, Button, Label, Input, Checkbox } from 'flowbite-svelte';
  </script>
  <div class="flex items-center justify-center h-screen">
  <Card>
    <form class="flex flex-col space-y-6" action="/">
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
    </form>
  </Card>
</div>