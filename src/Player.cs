public class Player
{
    public required String playerID;
    public required String Name;
    public required DateTime dateOfBirth;
    public required int Height;
    public required int Weight;

    // Get methods
    public String getPlayerName() { return Name; }
    public DateTime getPlayerDOB() { return dateOfBirth; }
    public int getPlayerHeight() { return Height; }
    public int getPlayerWeight() { return Weight; }

    // Set methods
    public void setPlayerName(String newName) { Name = newName; }
    public void setPlayerHeight(DateTime newDOB) { dateOfBirth = newDOB; }
    public void setPlayerHeight(int newHeight) { Height = newHeight; }
    public void setPlayerWeight(int newWeight) { Weight = newWeight; }
}
