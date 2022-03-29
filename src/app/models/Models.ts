export class Position{
  public latitude: string;
  public longitude: string;
}

export class Flat{
  public name: string;
  public rooms: string;
  public price: string;
  public location: Position;
  public features?: Array<string>;
  public image?: string;
}
