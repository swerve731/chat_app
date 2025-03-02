<script>
    import { Listgroup, ListgroupItem, Avatar } from 'flowbite-svelte';
    import { TrashBinSolid } from 'flowbite-svelte-icons';
  </script>
  
  <Listgroup active class="self-centerw-100">
    <h3 class="p-1 text-center text-xl font-medium text-gray-900 dark:text-white">User list</h3>
    <ListgroupItem class="text-base font-semibold gap-2">
      <Avatar src="/images/profile-picture-1.webp" size="xs" />Jese Leos
    </ListgroupItem>
    <ListgroupItem class="text-base font-semibold gap-2">
      <Avatar src="/images/profile-picture-2.webp" size="xs" />Robert Gouth
    </ListgroupItem>
    <ListgroupItem class="text-base font-semibold gap-2">
      <Avatar src="/images/profile-picture-3.webp" size="xs" />Bonnie Green
    </ListgroupItem>
    <a href="/" class="flex items-center p-3 text-sm font-medium text-red-600 bg-gray-50 hover:bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 dark:text-red-500 hover:underline rounded-b-lg">
      <TrashBinSolid class="w-6 h-6 ms-1 me-2" />
      Delete user
    </a>
  </Listgroup>