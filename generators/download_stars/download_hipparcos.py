from astroquery.vizier import Vizier
import astropy.table


def main():
    # Initialize Vizier query, specifying no row limit to get all entries
    Vizier.ROW_LIMIT = -1  # No limit
    
    # Query the Hipparcos catalogue (ID: I/239/hip_main)
    hipparcos_catalogue_id = "I/239/hip_main"
    catalogues = Vizier.get_catalogs(hipparcos_catalogue_id)
    
    if len(catalogues) == 0:
        print("No catalogues found")
        return

    # Extract the main catalogue
    hipparcos_catalogue = catalogues[0]
    (grammar_be, grammar_s) = ("are", "s")
    if len(catalogues) == 1:
        (grammar_be, grammar_s) = ("is", "")
    print(f"There {grammar_be} {len(catalogues)} available catalogue{grammar_s}")

    # Save to a CSV file
    hipparcos_catalogue.write("hipparcos_catalogue.csv", format="csv", overwrite=True)

    print(f"Downloaded {len(hipparcos_catalogue)} stars from the Hipparcos catalogue.")

if __name__ == "__main__":
    main()
