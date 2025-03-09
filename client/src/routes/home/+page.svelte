<script>
  import { Search, Table, TableBody, TableBodyCell, TableBodyRow } from 'flowbite-svelte';
  import { Avatar } from 'flowbite-svelte';

  let searchTerm = "";
  let items = [
    { id: 1, name: 'John Doe', contact: 'johndoe@example.com', lastMessage: '10m ago', avatar: '/images/profile1.webp' },
    { id: 2, name: 'Jane Smith', contact: '+1 234 567 8901', lastMessage: '1h ago', avatar: '/images/profile2.webp' },
    { id: 3, name: 'Alice Johnson', contact: 'alice.j@example.com', lastMessage: '2d ago', avatar: '/images/profile3.webp' },
    { id: 4, name: 'Bob Williams', contact: '+44 20 7946 0958', lastMessage: '1w ago', avatar: '/images/profile4.webp' }
  ];

  let filteredItems = items.filter(item => 
    item.name.toLowerCase().includes(searchTerm.toLowerCase()) || 
    item.contact.toLowerCase().includes(searchTerm.toLowerCase())
  );
</script>
<div class="relative w-150 mx-auto mb-3">
<input 
  type="text" 
  placeholder="Search..." 
  bind:value={searchTerm} 
  class="w-full p-2 border rounded-md mb-2 dark:bg-gray-800 dark:text-white"
>
</div>

<Table hoverable={true}>
  <TableBody tableBodyClass="divide-y">
    {#each filteredItems as item}
    <TableBodyRow class="flex items-center justify-between py-2 px-4">
      <div class="flex itmes-center gap-3">
        <Avatar class="w-15 h-15"src={item.avatar} dot={{ placement: 'bottom-right', color: 'green' }} />
        <div>
          <p class="text-lg font-semibold">{item.name}</p>
          <p class="text-base text-gray-500 dark:text-gray-400">{item.contact}</p>
        </div>

      </div>
      <div class="text-sm text-gray-500 dark:text-gray-400">
        {item.lastMessage}
      </div>
    </TableBodyRow>
    {/each}
  </TableBody>
</Table>