<script lang="ts">
  import { onMount } from "svelte";

  let name = "";
  let email = "";
  let password = "";
  let users: User[] = [];
  let groupName = "";
  let groups: Group[] = [];
  let description = "";
  let amount = "";
  let payerId = "";
  let selectedParticipants: number[] = [];
  let selectedOwner = "";
  let selectedGroupId = "";
  let expenses: string[] = [];

  // Interfaces
  export interface Group {
    id?: number;
    name: string;
    owner: number;
    members?: User[];
    expenses?: Expense[];
  }

  export interface User {
    id?: number;
    name: string;
    email: string;
    password: string;
  }

  export interface Expense {
    id: number;
    description?: string;
    amount: number;
    payer: User;
    participants: User[];
    date: string;
  }

  export interface Transaction {
    id: number;
    payer: User;
    receiver: User;
    amount: number;
    date: string;
  }

  // Add User
  async function addUser() {
    const response = await fetch("http://localhost:3000/user/create_user", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name, email, password }),
    });
    const data = await response.text();
    alert(data);
    await fetchUsers();
  }

  // Fetch Users
  async function fetchUsers() {
    const response = await fetch("http://localhost:3000/user/get_users");
    const json: User[] = await response.json();
    users = json;
  }

  // Create Group
  async function createGroup() {
    const response = await fetch("http://localhost:3000/group/create_group", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name: groupName, owner: Number(selectedOwner) }),
    });
    const data = await response.text();
    alert(data);
    await fetchGroups();
  }

  // Fetch Groups
  async function fetchGroups() {
    const response = await fetch("http://localhost:3000/group/get_groups", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ owner: Number(selectedOwner) }),
    });
    const json: Group[] = await response.json();
    groups = json;
  }

  // Add Expense
  async function addExpense() {
    const payer = users.find((user) => user.id === Number(payerId));
    const participants = users.filter((user) =>
      selectedParticipants.includes(user.id!)
    );

    // if (payer === undefined || participants.length === 0 || !selectedGroupId) {
    // 	alert('Please select a valid payer, participants, and group.');
    // 	return;
    // }

    const response = await fetch("http://localhost:3000/group/add_expense", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        group_info: { owner: Number(payerId), group_id: selectedGroupId },
        expense: {
          id: Math.floor(Math.random() * 10000),
          description,
          amount: parseFloat(amount),
          payer,
          participants,
          date: new Date().toISOString().split("T")[0],
        },
      }),
    });
    const data = await response.text();
    alert(data);
    await fetchExpenses();
  }

  // Fetch Expenses
  async function fetchExpenses() {
    // if (selectedOwner === undefined || !selectedGroupId) {
    // 	alert('Please select an owner and a group before fetching expenses.');
    // 	return;
    // }

    const response = await fetch("http://localhost:3000/group/calculate", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        owner: Number(selectedOwner),
        group_id: Number(selectedGroupId),
      }),
    });
    const json = await response.json();
    expenses = json.transactions.map(
      (transaction: Transaction) =>
        `Payer: ${transaction.receiver.name} -> Receiver: ${transaction.payer.name}, Amount: ${transaction.amount}`
    );
  }

  onMount(() => {
    fetchUsers();
    fetchGroups();
  });
</script>

<section class="container">
  <!-- Add User -->
  <div class="card">
    <h2>Add User</h2>
    <input type="text" bind:value={name} placeholder="Enter user name" />
    <input type="email" bind:value={email} placeholder="Enter email" />
    <input type="password" bind:value={password} placeholder="Enter password" />
    <button on:click={addUser}>Add User</button>
  </div>

  <!-- Users List -->
  <div class="card">
    <h2>Users List</h2>
    <button on:click={fetchUsers}>Refresh Users</button>
    <ul>
      {#each users as user}
        <li>{user.id}: {user.name} ({user.email})</li>
      {/each}
    </ul>
  </div>

  <!-- Create Group -->
  <div class="card">
    <h2>Create Group</h2>
    <input type="text" bind:value={groupName} placeholder="Enter group name" />
    <label>Owner:</label>
    <select bind:value={selectedOwner}>
      <option value="" disabled>Select Owner</option>
      {#each users as user}
        <option value={user.id}>{user.name} ({user.email})</option>
      {/each}
    </select>
    <button on:click={createGroup}>Create Group</button>
  </div>

  <!-- Groups List -->
  <div class="card">
    <h2>Groups List</h2>
    <button on:click={fetchGroups}>Refresh Groups</button>
    <ul>
      {#each groups as group}
        <li>{group.id}: {group.name}</li>
      {/each}
    </ul>
  </div>

  <!-- Add Expense -->
  <div class="card">
    <h2>Add Expense</h2>
    <input
      type="text"
      bind:value={description}
      placeholder="Expense description"
    />
    <input type="number" bind:value={amount} placeholder="Amount" />

    <!-- Select Payer -->
    <label>Payer:</label>
    <select bind:value={payerId}>
      <option value="" disabled>Select Payer</option>
      {#each users as user}
        <option value={user.id}>{user.name} ({user.email})</option>
      {/each}
    </select>

    <!-- Select Participants -->
    <!-- Select Participants -->
    <label>Participants:</label>
    <div class="participants-container">
      {#each users as user}
        <div class="participant">
          <input
            type="checkbox"
            bind:group={selectedParticipants}
            value={user.id}
            id={`participant-${user.id}`}
          />
          <label for={`participant-${user.id}`}>
            {user.name} ({user.email})
          </label>
        </div>
      {/each}
    </div>

    <label>Select Group:</label>
    <select bind:value={selectedGroupId}>
      <option value="" disabled>Select Group</option>
      {#each groups as group}
        <option value={group.id}>{group.name}</option>
      {/each}
    </select>

    <button on:click={addExpense}>Add Expense</button>
  </div>

  <!-- Expenses List -->
  <div class="card">
    <h2>Expenses List</h2>
    <label>Select Owner:</label>
    <select bind:value={selectedOwner}>
      <option value="" disabled>Select Owner</option>
      {#each users as user}
        <option value={user.id}>{user.name}</option>
      {/each}
    </select>

    <label>Select Group:</label>
    <select bind:value={selectedGroupId}>
      <option value="" disabled>Select Group</option>
      {#each groups as group}
        <option value={group.id}>{group.name}</option>
      {/each}
    </select>

    <button on:click={fetchExpenses}>Fetch Expenses</button>
    <ul>
      {#each expenses as expense}
        <li>{expense}</li>
      {/each}
    </ul>
  </div>
</section>

<style>
  .container {
    display: flex;
    flex-wrap: wrap;
    gap: 30px;
    padding: 30px;
  }

  .card {
    background: white;
    padding: 35px;
    border-radius: 8px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    width: 300px;
  }

  .card h2 {
    margin-bottom: 10px;
  }

  .card input,
  select,
  button {
    width: 90%;
    margin: 5px 0;
    padding: 8px;
    border: 1px solid #ccc;
    border-radius: 4px;
  }

  button {
    background-color: #4caf50;
    color: white;
    cursor: pointer;
    border: none;
  }

  button:hover {
    background-color: #45a049;
  }
  .participants-container {
    display: flex;
    flex-wrap: wrap;
    gap: 3px;
    border: 1px solid #ccc;
    border-radius: 4px;
    background-color: #f9f9f9;
  }

  .participant {
    display: flex;
    align-items: center;
  }
</style>
