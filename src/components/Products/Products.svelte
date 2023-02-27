<script lang="ts">
  import type {
    Brand,
    BrandNames,
    Category,
    CategoryNames,
    Product,
    Supplier,
    SupplierNames,
  } from "src/lib/types";
  import GroupBox from "../../lib/GroupBox.svelte";
  import Card from "../../lib/Card.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button, Modal, TextInput } from "carbon-components-svelte";
  import NumberInput from "../../lib/NumberInput.svelte";
  import Search from "../../lib/Search.svelte";

  export let products: Product[];
  export let brands: Brand[];
  export let suppliers: Supplier[];
  export let categories: Category[];
  export let brandSelectedRowIds: number[][] = [];
  export let supplierSelectedRowIds: number[][] = [];
  export let categorySelectedRowIds: number[][] = [];
  let brandHeaders = [
    { key: "name", value: "Name" },
    { key: "id", value: "ID" },
  ];
  let supplierHeaders = [
    { key: "name", value: "Name" },
    { key: "id", value: "ID" },
  ];
  let categoryHeaders = [
    { key: "name", value: "Name" },
    { key: "id", value: "ID" },
  ];

  export let brand_open: boolean[] = products.map((product) => false);
  export let brand_text: string = "";
  export let brand_rows: BrandNames[] = [];
  (async () => {
    await invoke("brand_names").then(
      (brands: BrandNames[]) => (brand_rows = brands)
    );
  })();
  export let brand_search: () => void = async () => {
    console.log("search");
    await invoke("sort_brands", { brandNames: brand_rows, search: brand_text })
      .then((res: BrandNames[]) => (console.log(res), (brand_rows = res)))
      .catch((e) => console.log(e));
  };

  export let supplier_open: boolean[] = products.map((product) => false);
  export let supplier_text: string = "";
  export let supplier_rows: BrandNames[] = [];
  (async () => {
    await invoke("supplier_names").then(
      (suppliers: SupplierNames[]) => (supplier_rows = suppliers)
    );
  })();
  export let supplier_search: () => void = async () => {
    console.log("search");
    await invoke("sort_suppliers", {
      supplierNames: supplier_rows,
      search: supplier_text,
    })
      .then((res: SupplierNames[]) => (console.log(res), (supplier_rows = res)))
      .catch((e) => console.log(e));
  };

  export let category_open: boolean[] = products.map((product) => false);
  export let category_text: string = "";
  export let category_rows: BrandNames[] = [];
  (async () => {
    await invoke("category_names").then(
      (categories: CategoryNames[]) => (category_rows = categories)
    );
  })();
  export let category_search: () => void = async () => {
    console.log("search");
    await invoke("sort_categories", {
      categoryNames: category_rows,
      search: category_text,
    })
      .then((res: CategoryNames[]) => (console.log(res), (category_rows = res)))
      .catch((e) => console.log(e));
  };

  async function setBrands(product: Product, selectedRows: number[]) {
    console.log(selectedRows);
    if (product.getBrand() != null) {
      if (product.getBrand().products != null) {
        let brand = product.getBrand();
        brand.products = brand.products.filter((p) => p != product.id);

        await invoke("save_brand", { brand });
        let index = brands.findIndex((b) => b.id == brand.id);
        brands[index] = brand;
      }
    }
    let index = brands.findIndex((brand) => selectedRows[0] == brand.id);
    console.log(index);
    brands[index].products.push(product.id);
    console.log(brands);
    await invoke("save_brand", { brand: brands[index] });
    brands = brands;
    product.getBrand = product.getBrand;
  }

  async function setCategories(product: Product, selectedRows: number[]) {
    console.log(selectedRows);
    product.getCategories().forEach(async (category) => {
      if (category.products != null) {
        let c = category;
        c.products = c.products.filter((p) => p != product.id);

        await invoke("save_category", { category: c });
        let index = categories.findIndex((b) => b.id == c.id);
        categories[index] = c;
      }
    });
    selectedRows.forEach(async (row) => {
      let index = categories.findIndex((category) => row == category.id);
      console.log(index);
      categories[index].products.push(product.id);
      console.log(categories);
      await invoke("save_category", { category: categories[index] });
      categories = categories;
    });
    product.getCategories = product.getCategories;
  }

  async function setSuppliers(product: Product, selectedRows: number[]) {
    console.log(selectedRows);
    product.getSuppliers().forEach(async (supplier) => {
      if (supplier.products != null) {
        let s = supplier;
        s.products = s.products.filter((p) => p != product.id);

        await invoke("save_supplier", { supplier: s });
        let index = suppliers.findIndex((b) => b.id == s.id);
        suppliers[index] = s;
      }
    });
    selectedRows.forEach(async (row) => {
      let index = suppliers.findIndex((supplier) => row == supplier.id);
      console.log(index);
      suppliers[index].products.push(product.id);
      console.log(suppliers);
      await invoke("save_supplier", { supplier: suppliers[index] });
      suppliers = suppliers;
    });
    product.getCategories = product.getCategories;
  }

  async function remove(index: number) {
    let product = products.splice(index, 1)[0];
    console.log(product.id);
    await invoke("remove_product", { id: product.id })
      .then(() => (products = products))
      .catch((e) => console.log(e));
    products = products;
  }
  async function new_product() {
    await invoke("new_product")
      .then(
        (product: Product) =>
          (products = [
            ...products,
            {
              ...product,
              getSuppliers: () => [],
              getCategories: () => [],
              getBrand: () => null,
            },
          ])
      )
      .catch((e) => console.log(e));
  }

  let editing: boolean[] = products.map((product) => false);
  function edit(index: number) {
    editing[index] = true;
  }
  async function save(index: number) {
    await invoke("save_product", { product: products[index] })
      .then(() => (editing[index] = false))
      .catch((e) => console.log(e));
  }
  let chooseBrand = false;
</script>

<div class="products">
  {#each products as product, index (product.id)}
    <Card
      editable={true}
      edit={() => edit(index)}
      save={() => save(index)}
      remove={() => remove(index)}
    >
      {#if !editing[index]}
        <GroupBox label="name" value={product.name} />
        <GroupBox label="upc" value={product.upc} />
        <GroupBox label="amount" value={product.amount} />
        <GroupBox label="buying level" value={product.buyLevel} />
        <GroupBox label="description" value={product.description} />
        <GroupBox label="cost price" value={product.costPrice} />
        <GroupBox label="selling price" value={product.sellingPrice} />
        <GroupBox
          label="brand"
          value={product.getBrand() ? product.getBrand().name : "none"}
        />
        <GroupBox
          label="suppliers"
          value={product.getSuppliers().map((supplier) => supplier.name)}
        />
        <GroupBox
          label="categories"
          value={product.getCategories().map((category) => category.name)}
        />
      {:else}
        <TextInput labelText="Name" bind:value={product.name} />
        <TextInput labelText="UPC" bind:value={product.upc} />
        <NumberInput label="Amount" bind:value={product.amount} />
        <NumberInput label="Buying Level" bind:value={product.buyLevel} />
        <TextInput labelText="Description" bind:value={product.description} />
        <TextInput labelText="Cost Price" bind:value={product.costPrice} />
        <TextInput
          labelText="Selling Price"
          bind:value={product.sellingPrice}
        />
        <div>
          <div>-{product.getBrand() ? product.getBrand().name : "none"}-</div>
          <Button on:click={() => (brand_open[index] = true)} label="brand"
            >Add Brand</Button
          >
        </div>
        <div>
          <div>-{product.getSuppliers().map((supplier) => supplier.name)}-</div>
          <Button
            on:click={() => (supplier_open[index] = true)}
            label="supplier">Manage Suppliers</Button
          >
        </div>
        <div>
          <div>
            -{product.getCategories().map((category) => category.name)}-
          </div>
          <Button
            on:click={() => (category_open[index] = true)}
            label="category">Manage Categories</Button
          >
        </div>
        {#if brand_open[index] == true}
          <Search
            bind:open={brand_open[index]}
            bind:text={brand_text}
            bind:rows={brand_rows}
            search={brand_search}
            okay={() => setBrands(products[index], brandSelectedRowIds[index])}
            bind:selectedRowIds={brandSelectedRowIds[index]}
            headers={brandHeaders}
          />
        {/if}
        {#if supplier_open[index] == true}
          <Search
            bind:open={supplier_open[index]}
            bind:text={supplier_text}
            bind:rows={supplier_rows}
            search={supplier_search}
            batchSelection={true}
            radio={false}
            okay={() =>
              setSuppliers(products[index], supplierSelectedRowIds[index])}
            bind:selectedRowIds={supplierSelectedRowIds[index]}
            headers={supplierHeaders}
          />
        {/if}
        {#if category_open[index] == true}
          <Search
            bind:open={category_open[index]}
            bind:text={category_text}
            radio={false}
            batchSelection={true}
            bind:rows={category_rows}
            search={category_search}
            okay={() =>
              setCategories(products[index], categorySelectedRowIds[index])}
            bind:selectedRowIds={categorySelectedRowIds[index]}
            headers={categoryHeaders}
          />
        {/if}
      {/if}
    </Card>
  {/each}
  <Button style="width: 500px;" kind="tertiary" on:click={new_product}
    >+ Add</Button
  >
</div>

<style lang="scss">
  .products {
    width: 100%;
    padding: 40px;
    display: flex;
    align-items: center;
    flex-direction: column-reverse;
    gap: 40px;
  }
</style>
