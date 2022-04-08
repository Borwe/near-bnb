export class House{
  name: string;
  price: string;
  location: string;
  features: string;
  image: string;
}

export class HouseInfo{
  name: string;
  price: string;
  location: string;
  features: Array<string>;
  image: string
}

export class Date{
  day: number; month: number; year: number
}

export class HouseName{ house_name: string }
