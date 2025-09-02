package com.github.noreply.users.stellar_guesser;

import android.app.NativeActivity;
import android.graphics.Color;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.view.Window;
import android.view.WindowManager;

import androidx.annotation.Nullable;
import androidx.core.graphics.Insets;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowCompat;
import androidx.core.view.WindowInsetsCompat;

public class MainActivity extends NativeActivity {
    private static final String TAG = "CustomSGMainActivity";

    @Override
    protected void onCreate(@Nullable Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        Log.i(TAG, "onCreate called - Applying manual padding approach");

        final Window window = getWindow();

        // **Step 1: Explicitly go edge-to-edge to gain control.**
        // This ensures we receive the inset data.
        WindowCompat.setDecorFitsSystemWindows(window, false);

        // **Step 2 (Optional but Recommended): Make system bars transparent.**
        // This prevents solid black or white bars from appearing in the inset areas
        // before your view is padded.
        window.setStatusBarColor(Color.TRANSPARENT);
        window.setNavigationBarColor(Color.TRANSPARENT);

        // As before, this is a good secondary flag to set for clarity.
        WindowManager.LayoutParams lp = window.getAttributes();
        lp.layoutInDisplayCutoutMode = WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_NEVER;
        window.setAttributes(lp);

        // **Step 3: Get the root content view.**
        // This is the container we will apply padding to.
        final View content = findViewById(android.R.id.content);

        // **Step 4: Set a listener to apply the insets as PADDING.**
        ViewCompat.setOnApplyWindowInsetsListener(content, (v, insets) -> {
            Insets sysBars = insets.getInsets(WindowInsetsCompat.Type.systemBars());
            Insets cutout = insets.getInsets(WindowInsetsCompat.Type.displayCutout());

            // Calculate the total safe area insets on all sides
            int top = Math.max(sysBars.top, cutout.top);
            int left = Math.max(sysBars.left, cutout.left);
            int right = Math.max(sysBars.right, cutout.right);
            int bottom = Math.max(sysBars.bottom, cutout.bottom);

            // Apply the calculated insets as padding to the root content view.
            // This will constrain any children, including the NativeActivity's SurfaceView.
            v.setPadding(left, top, right, bottom);

            Log.i(TAG, "Applied padding to content view: Top=" + top + ", Left=" + left + ", Bottom=" + bottom);

            // Tell the system that we've handled the insets.
            return WindowInsetsCompat.CONSUMED;
        });
    }
}