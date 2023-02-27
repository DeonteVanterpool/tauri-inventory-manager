export class Product {
    id: number;
    name: string;
    upc: string;
    description: string;
    buyLevel: number;
    costPrice: string;
    sellingPrice: string;
    amount: number;
    case_size: number;
    measureByWeight: boolean;
    getBrand: () => Brand;
    getCategories: () => Category[];
    getSuppliers: () => Supplier[];
}

export class Brand {
    id: number;
    name: string;
    products: number[];
}

export class Category {
    id: number;
    name: string;
    products: number[];
}

export class Supplier {
    id: number;
    name: string;
    phoneNumber: string;
    email: string;
    products: number[];
}

export class ReceivedOrder {
    id: number;
    received: string;
    product_id: number;
    gross_amount: number;
    actually_received: number;
    damaged: number;
    getProduct: () => Product;
}

export class PendingOrder {
    id: number;
    product: number;
    amount: number;
    getProduct: () => Product;
}

export class ProductNames {
    name: string;
    upc: string;
    id: number;
}

export class BrandNames {
    name: string;
    id: number;
}

export class SupplierNames {
    name: string;
    id: number;
}

export class CategoryNames {
    name: string;
    id: number;
}
