<script lang="ts">
  import Dashboard from "./components/Dashboard/Dashboard.svelte";
  import Header2 from "./components/Header2.svelte";
  import SideBar from "./components/SideBar.svelte";
  import Header1 from "./components/Header1.svelte";
  import type {
    PendingOrder,
    ReceivedOrder,
    Brand,
    Category,
    Product,
    Supplier,
  } from "./lib/types";
  import { ContentSwitcher, Switch } from "carbon-components-svelte";
  import LogIn from "./LogIn.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import Products from "./components/Products/Products.svelte";
  import Brands from "./components/Brands/Brands.svelte";
  import Suppliers from "./components/Suppliers/Suppliers.svelte";
  import Categories from "./components/Categories/Categories.svelte";
  import Orders from "./components/Orders/Orders.svelte";
  // test
  // test
  let sideBarItems = [
    "dashboard",
    "orders",
    "products",
    "categories",
    "suppliers",
    "brands",
    "", // settings
    "log out",
  ];

  let selected: number = 0;

  let products: Product[] = [];
  $: lowInventory = products.filter(
    (product) => product.amount < product.buyLevel
  );
  let brands: Brand[] = [];
  let categories: Category[] = [];
  let suppliers: Supplier[] = [];

  $: changeSecondHeader(selected);

  let receivedOrders: ReceivedOrder[] = [];
  let pendingOrders: PendingOrder[] = [];

  function changeSecondHeader(selected: number) {
    secondHeaderText = "";
    if (selected == 7) {
      loggedIn = false;
    }
  }

  let orderIndex: number = 0;

  $: selectedText = sideBarItems[selected];
  let dashboardIndex: number = 0;
  let secondHeaderText: string = "";

  function getProduct(id: number): Product {
    return products.filter((p) => p.id == id)[0];
  }

  function onOrderChange(index: number) {
    if (index == 0) {
      secondHeaderText = "Pending";
    } else {
      secondHeaderText = "Received";
    }
  }
  let loggedIn: boolean = false;

  async function logIn(username: string, password: string) {
    console.log("not invoked");

    await invoke("log_in", { username, password })
      .then(() => {
        loggedIn = true;
        console.log("invoked");
      })
      .catch((e) => {
        loggedIn = false;
        console.log(e);
      });

    let prods: Product[] = await invoke("get_products", {
      limit: 10,
      offset: 0,
    }); // incomplete products

    products = prods.map((product: any) => ({
      ...product,
      getSuppliers: () =>
        suppliers.filter((supplier) => supplier.products.includes(product.id)),
      getCategories: () =>
        categories.filter((category) => category.products.includes(product.id)),
      getBrand: () =>
        brands.filter((brand) => brand.products.includes(product.id))
          ? brands.find((brand) => brand.products.includes(product.id))
          : null,
    }));

    lowInventory = products.filter(
      (product) => product.amount < product.buyLevel
    );
    console.log(
      products.filter((product) => product.amount < product.buyLevel)
    );
    console.log(lowInventory);
    console.log(products);

    await invoke("get_brands", {
      limit: 10,
      offset: 0,
    })
      .then((b: Brand[]) => (brands = b))
      .catch((e) => console.log(e, "couldn't get brands"));

    let rec: ReceivedOrder[] = [];

    await invoke("get_received_orders", {
      limit: 10,
      offset: 0,
    })
      .then((o: ReceivedOrder[]) => (rec = o))
      .catch((e) => console.log(e, "couldn't get suppliers"));
    rec.map(
      (o) => (
        console.log("a"),
        console.log(o),
        console.log(products.filter((p) => p.id == o.product_id)),
        (o.product_id = products.filter((p) => p.id == o.product_id)[0].id),
        (o.getProduct = function () {
          return getProduct(this.product_id);
        })
      )
    );
    // hello
    receivedOrders = rec;

    let pend: PendingOrder[] = [];

    console.log(products[0].id);

    await invoke("get_pending_orders", {
      limit: 10,
      offset: 0,
    })
      .then((o: PendingOrder[]) => (pend = o))
      .catch((e) => console.log(e, "couldn't get suppliers"));
    pend.map(
      (o) => (
        console.log(o.product),
        console.log(products.filter((p) => p.id == o.product)),
        console.log(products.filter((p) => p.id == o.product)[0]),
        (o.product = products.filter((p) => p.id == o.product)[0].id),
        (o.getProduct = function () {
          return getProduct(this.product);
        })
      )
    );
    pendingOrders = pend;
    //comment

    await invoke("get_suppliers", { limit: 10, offset: 0 })
      .then((s: Supplier[]) => (suppliers = s))
      .catch((e) => console.log(e, "couldn't get suppliers"));

    await invoke("get_categories", { limit: 10, offset: 0 })
      .then((s: Category[]) => (categories = s))
      .catch((e) => console.log(e, "couldn't get categories"));
  }
</script>

<main class="container">
  {#if !loggedIn}
    <LogIn {logIn} />
  {:else}
    <SideBar {sideBarItems} bind:selected />
    <Header1 headerText={selectedText} />
    <Header2 headerText={secondHeaderText} />
    <div class="content">
      {#if selected == 0}
        <Dashboard bind:lowInventory />
      {:else if selected == 1}
        <ContentSwitcher
          bind:selectedIndex={orderIndex}
          on:change={() => {
            onOrderChange(orderIndex);
          }}
        >
          <Switch text="Pending" />
          <Switch text="Received" />
        </ContentSwitcher>
        <Orders
          bind:pendingOrders
          bind:receivedOrders
          bind:orderIndex
          firstProduct={products[0].id}
          {getProduct}
        />
      {:else if selected == 2}
        <Products bind:products bind:brands bind:categories bind:suppliers />
      {:else if selected == 3}
        <Categories bind:categories />
      {:else if selected == 4}
        <Suppliers bind:suppliers />
      {:else if selected == 5}
        <Brands bind:brands />
      {/if}
    </div>
  {/if}
</main>

<style>
  .container {
    display: flex;
    overflow-y: 0;
    padding: 0;
    height: 100vh;
  }
  .container .content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: start;
    margin-left: 300px;
    margin-top: 200px;
    height: calc(100vh - 200px);
    overflow-y: auto;
    background-color: white;
  }
</style>
