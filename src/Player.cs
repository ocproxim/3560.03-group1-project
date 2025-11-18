public class Player
{
    public required int playerID;
    public required String Name;
    public required String dateOfBirth;
    public required int Height;
    public required int Weight;

    // Get methods
    public String getPlayerName() { return Name; }
    public DateTime getPlayerDOB() { return DateTime.Parse(dateOfBirth); }
    public int getPlayerHeight() { return Height; }
    public int getPlayerWeight() { return Weight; }

    // Set methods
    public void setPlayerName(String newName) { Name = newName; }
    public void setPlayerHeight(DateTime newDOB) { dateOfBirth = newDOB.ToString(); }
    public void setPlayerHeight(int newHeight) { Height = newHeight; }
    public void setPlayerWeight(int newWeight) { Weight = newWeight; }
}
