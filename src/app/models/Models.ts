export class Position{
  public latitude: string;
  public longitude: string;
}

export class Flat{
  public name: string;
  public rooms: number;
  public price: string;
  public location: Position;
  public features: Array<string> | null;
  public image: string | null;
}
