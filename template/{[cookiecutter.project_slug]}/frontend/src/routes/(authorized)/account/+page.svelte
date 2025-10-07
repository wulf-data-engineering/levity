<script lang="ts">
    let { data } = $props();
    let currentUser = data.currentUser!;
</script>

<svelte:head>
	<title>Protected Page</title>
	<meta name="description" content="Protected route" />
</svelte:head>

<div>
	<h1>Page for signed in users</h1>
    <p>
        You are signed in as {currentUser.signInDetails?.loginId}
        <small>{currentUser.userId}</small>.
    </p>
</div>
