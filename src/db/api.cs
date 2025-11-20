using Microsoft.AspNetCore.Mvc;
using System.Collections.Generic;
using Search;

[ApiController]
[Route("api/[controller]")]
public class SportsController : ControllerBase
{
    [HttpGet]
    public ActionResult<List<string>> GetSportNames(DBConnection db)
    {
        // Create a DBConnection and fetch the sport names using the existing method
        List<string> names = db.FetchSports();
        return Ok(names);
    }
}