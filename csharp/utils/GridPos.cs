namespace adventofcode.utils;

public class GridPos(int col, int row)
{
    public static readonly GridPos MoveU = new(0, 1);
    public static readonly GridPos MoveD = new(0, -1);
    public static readonly GridPos MoveR = new(1, 0);
    public static readonly GridPos MoveL = new(-1, 0);
    public static readonly GridPos MoveUr = new(1, 1);
    public static readonly GridPos MoveUl = new(-1, 1);
    public static readonly GridPos MoveDl = new(-1, -1);
    public static readonly GridPos MoveDr = new(1, -1);
    public static readonly GridPos[] AllOrthogonal = [MoveU, MoveR, MoveD, MoveL];
    public static readonly GridPos[] AllSurrounding = [MoveU, MoveUr, MoveR, MoveDr, MoveD, MoveDl, MoveL, MoveUl];

    public int Col { get; private set; } = col;
    public int Row { get; private set; } = row;

    public GridPos Distance(GridPos other) => new(Col - other.Col, Row - other.Row);

    public void MoveBy(GridPos delta)
    {
        Col += delta.Col;
        Row += delta.Row;
    }

    public GridPos Add(GridPos other) => new(Col + other.Col, Row + other.Row);

    public override string ToString() => $"({Col},{Row})";

    public bool InBounds(int width, int height) =>
        Col >= 0 && Col < width && Row >= 0 && Row < height;

    public override bool Equals(object? obj)
    {
        return obj is GridPos pos && pos.Col == Col && pos.Row == Row;
    }

    protected bool Equals(GridPos other)
    {
        return Col == other.Col && Row == other.Row;
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(Col, Row);
    }
}